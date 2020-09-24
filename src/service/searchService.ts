import * as fs from "fs";
import * as path from "path";
import * as yaml from "js-yaml";
import MarkdownHeaderDto from "../model/MarkdownHeaderDto";
import MarkdownHeader from "../model/MarkdownHeader";

const BASE_PATH = "./documents";
const HEADER_MATCHER = /^---(.*)---/s

class SearchService {
    public getAllHeaders(baseHost: string) {
        return fs.readdirSync(BASE_PATH)
            .map(fileName => {
                const fileContent = fs.readFileSync(path.join(BASE_PATH, fileName)).toString();
                const stringHeader = HEADER_MATCHER.exec(fileContent)[1];
                const fileHeader = yaml.safeLoad(stringHeader) as MarkdownHeader;
                return {
                    title: fileHeader.title,
                    type: fileHeader.type,
                    description: fileHeader.description,
                    author: fileHeader.author,
                    date: new Date(fileHeader.date),
                    filename: fileName,
                    url: path.join(baseHost, "files", fileName)
                } as MarkdownHeaderDto;
            });
    }

    public filter(pageId: number, itemsPerPage: number, type: "ARTICLE" | "BLOG" | "DOCUMENTATION", query: string, baseHost: string) {
        return this.getAllHeaders(baseHost)
            .sort(header => header.date.getTime())
            .filter(header => header.type == type ) // Filter by document type
            .filter(header => {
                const regex = RegExp(query, "i");
                return regex.test(header.title) || regex.test(header.description);
            }) // Flter with query
            .filter((header, index) => index >= itemsPerPage * (pageId - 1) && index < itemsPerPage * pageId) // Filter by page
    }
}

const searchService = new SearchService();

export default searchService;
