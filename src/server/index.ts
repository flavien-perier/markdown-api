import { OpenApiValidator } from "express-openapi-validator";
import * as express from "express";
import * as compression from "compression";
import * as bodyParser from "body-parser";
import * as helmet from "helmet";
import * as http from "http";
import * as yaml from "js-yaml";
import * as fs from "fs";
import configuration from "../service/configuration";
import logger from "../service/logger";
import HttpError from "../error/HttpError";
import HttpInternalServerError from "../error/HttpInternalServerError";
import HttpNotFoundError from "../error/HttpNotFoundError";
import BadRequestInformations from "../model/BadRequestInformations";
import ping from "./ping";

const _logger = logger("server");
const app = express();

app.use(bodyParser.json());
app.use(helmet());
app.use(compression());

// log request
app.use((req, res, next) => {
    const ip = req.headers["x-real-ip"] || req.connection.remoteAddress;
    const userAgent = req.get("User-Agent");
    const method = req.method;
    const url = req.originalUrl;

    _logger.http(`${method}: ${url}`, {ip, userAgent, method, url});
    next();
});

const server: Promise<http.Server> = new OpenApiValidator({
    apiSpec: yaml.safeLoad(fs.readFileSync("swagger.yaml", "utf8")) as any,
    validateRequests: true,
    validateResponses: true,
    validateSecurity: true
}).install(app).then(() => {
    // include rooters
    app.use("/ping", ping)

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
            res.status(err.status).json(new BadRequestInformations(err.message, err.errors));
        } else {
            _logger.error("Internal server error", {errorMessage: err.message || err});
            new HttpInternalServerError("Internal server error").apply(res);
        }
    });

    return app.listen(configuration.port, () => {
        _logger.info(`Application start on port ${configuration.port} with id "${configuration.nodeId}"`, {port: configuration.port, nodeId: configuration.nodeId});
    });
});

export { server, app };
