import * as express from "express";
import * as path from "path";

const router = express.Router();

router.get("/:fileName", (req, res) => {
    res.setHeader("Content-Type", "text/markdown");
    res.sendFile(path.join(process.cwd(), "documents", req.params.fileName));
});

export default router;
