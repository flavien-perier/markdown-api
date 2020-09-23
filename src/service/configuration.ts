import * as yaml from "js-yaml";
import * as fs from "fs";
import * as crypto from "crypto";

class Configuration {
    private _applicationVersion: string;
    private _nodeId: string;
    private _logLevel: string;
    private _port: string;
    constructor() {
        const staticConfiguration: any = yaml.safeLoad(fs.readFileSync("configuration.yaml", "utf8"));

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
