import ArticleType from "./ArticleType";

export default interface MarkdownHeader {
    title: string;
    type: ArticleType;
    description: string;
    author: string;
    date: Date;
    url: string;
}
