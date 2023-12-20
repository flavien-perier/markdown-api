import * as OpenApiValidator from "express-openapi-validator";
import * as express from "express";
import * as bodyParser from "body-parser";
import * as helmet from "helmet";
import * as fs from "fs";
import * as yaml from "yaml";
import configuration from "./environment";
import logger from "./logger";
import HttpError from "../error/http.error";
import HttpInternalServerError from "../error/http-internal-server.error";
import HttpNotFoundError from "../error/http-not-found.error";
import BadRequestNformationsModel from "../model/bad-request-nformations.model";
import search from "../controller/search.controller";

const _logger = logger("server");
const app = express();

app.use(bodyParser.json());
app.use(helmet.default());

// log request
app.use((req, res, next) => {
    const ip = req.headers["x-real-ip"] || req.connection.remoteAddress;
    const userAgent = req.get("User-Agent");
    const method = req.method;
    const url = req.originalUrl;

    _logger.http(`${method}: ${url}`, {ip, userAgent, method, url});
    next();
});

// edit header
app.use((req, res, next) => {
    res.header("Access-Control-Allow-Origin", "*");
    next();
});

app.use(OpenApiValidator.middleware({
        apiSpec: yaml.parse(fs.readFileSync("swagger.yaml", "utf8")) as any,
        validateRequests: true, // (default)
        validateResponses: true, // false by default
        validateSecurity: false
}));

// include rooters
app.use(express.static("documents"));
app.use("/", search);

// default response
app.use((req, res, next) => {
    next(new HttpNotFoundError("Not found"));
});

// error catching
app.use((err, req, res, next) => {
    if (err instanceof HttpError) {
        err.apply(res);
    } else if (err.status && err.message && err.errors) {
        _logger.warn("Swagger compliance issue", {error: err.message});
        res.status(err.status).json(new BadRequestNformationsModel(err.message, err.errors));
    } else {
        _logger.error("Internal controller error", {errorMessage: err.message || err});
        new HttpInternalServerError("Internal controller error").apply(res);
    }
});

const server = app.listen(configuration.port, () => {
    _logger.info(`Application start on port ${configuration.port} with id "${configuration.nodeId}"`, {port: configuration.port, nodeId: configuration.nodeId});
});

export { server, app };
