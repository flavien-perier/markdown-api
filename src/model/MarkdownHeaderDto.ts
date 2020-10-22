export default interface MarkdownHeader {
    title: string;
    type: "ARTICLE" | "BLOG" | "DOCUMENTATION";
    categories: string[];
    description: string;
    author: string;
    date: string;
    fileName: string;
    url: string;
}
