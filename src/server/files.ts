import * as express from "express";
import fileService from "../service/fileService";

const router = express.Router();

router.get("/:fileName", (req, res) => {
    res.json({
        fileName: req.params.fileName
    });
});

export default router;
