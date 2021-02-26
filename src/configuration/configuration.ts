import * as yaml from "js-yaml";
import * as fs from "fs";
import * as crypto from "crypto";

class Configuration {
    private readonly _applicationVersion: string;
    private readonly _nodeId: string;
    private readonly _logLevel: string;
    private readonly _port: string;
    constructor() {
        const staticConfiguration: any = yaml.load(fs.readFileSync("configuration.yaml", "utf8"));

        this._applicationVersion = require("../../package.json").version
        this._nodeId = process.env.NODE_ID || staticConfiguration.application.nodeId || crypto.randomBytes(10).toString("hex");
        this._logLevel = process.env.LOG || staticConfiguration.application.logLevel;
        this._port = process.env.PORT || staticConfiguration.application.port;
    }

    public get nodeId() {
        return this._nodeId;
    }

    public get applicationVersion() {
        return this._applicationVersion;
    }

    public get logLevel() {
        return this._logLevel;
    }

    public get port() {
        return this._port;
    }
}

const configuration = new Configuration();

export default configuration;
