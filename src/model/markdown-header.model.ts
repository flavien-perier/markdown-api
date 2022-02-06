export default interface MarkdownHeaderModel {
    title: string;
    type: "ARTICLE" | "BLOG" | "DOCUMENTATION" | "WIKI";
    categories: string[];
    description: string;
    author: string;
    date: string;
}
