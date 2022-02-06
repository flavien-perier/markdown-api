export default interface MarkdownHeader {
    title: string;
    type: "ARTICLE" | "BLOG" | "DOCUMENTATION" | "WIKI";
    categories: string[];
    description: string;
    author: string;
    date: string;
    fileName: string;
    url: string;
}
