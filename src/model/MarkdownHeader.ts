export default interface MarkdownHeader {
    title: string;
    type: "ARTICLE" | "BLOG" | "DOCUMENTATION";
    description: string;
    author: string;
    date: string;
}
