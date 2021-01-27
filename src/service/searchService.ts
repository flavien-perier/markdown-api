import * as fs from "fs";
import * as path from "path";
import * as yaml from "js-yaml";
import MarkdownHeaderDto from "../model/MarkdownHeaderDto";
import MarkdownHeader from "../model/MarkdownHeader";
import SearchResultDto from "../model/SearchResultDto";

const BASE_PATH = "./documents";
const HEADER_MATCHER = /^---(.*?)---/s;
const MARKDOWN_FILE_MATCHER = /^.*\.md$/;

class SearchService {
    public getAllHeaders(baseHost: string) {
        return fs.readdirSync(BASE_PATH)
            .filter(fileName => fs.lstatSync(path.join(BASE_PATH, fileName)).isFile() && MARKDOWN_FILE_MATCHER.test(fileName))
            .map(fileName => {
                const fileContent = fs.readFileSync(path.join(BASE_PATH, fileName)).toString();
                const stringHeader = HEADER_MATCHER.exec(fileContent)[1];
                const fileHeader = yaml.safeLoad(stringHeader) as MarkdownHeader;
                return {
                    title: fileHeader.title,
                    type: fileHeader.type,
                    categories: fileHeader.categories || [],
                    description: fileHeader.description,
                    author: fileHeader.author,
                    date: new Date(fileHeader.date).toISOString(),
                    fileName: fileName,
                    url: path.join(baseHost, fileName)
                } as MarkdownHeaderDto;
            });
    }

    public filter(pageId: number, itemsPerPage: number, type: "ARTICLE" | "BLOG" | "DOCUMENTATION", categ: string, query: string, baseHost: string) : SearchResultDto {
        const files = this.getAllHeaders(baseHost)
            .sort((file1, file2) => new Date(file2.date).getTime() - new Date(file1.date).getTime())
            .filter(header => header.type == type ) // Filter by document type
            .filter(header => categ == null || header.categories.indexOf(categ) != -1) // Filter by category
            .filter(header => {
                const regex = RegExp(query, "i");
                return regex.test(header.title) || regex.test(header.description);
            }) // Flter with query
            .filter((header, index) => index >= itemsPerPage * (pageId - 1) && index < itemsPerPage * pageId); // Filter by page

        return {
            pages: Math.round((files.length / itemsPerPage) + 1),
            files
        };
    }
}

const searchService = new SearchService();

export default searchService;
