import MarkdownHeaderDto from "./markdown-header.dto";

export default interface SearchResult {
    pages: number;
    files: MarkdownHeaderDto[];
}
