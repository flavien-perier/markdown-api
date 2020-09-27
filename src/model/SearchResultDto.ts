import MarkdownHeaderDto from "./MarkdownHeaderDto";

export default interface SearchResult {
    pages: number;
    files: MarkdownHeaderDto[];
}
