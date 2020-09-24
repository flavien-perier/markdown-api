import * as express from "express";
import searchService from "../service/searchService";

const router = express.Router();

router.get("/", (req, res) => {
    res.json(searchService.filter(
        parseInt(req.query.p as string) || 1,
        parseInt(req.query.n as string) || 10,
        (req.query.type as string || "ARTICLE") as "ARTICLE" | "BLOG" | "DOCUMENTATION",
        req.query.q as string,
        req.headers["host"]
    ));
});

export default router;
