import * as winston from "winston";
import configuration from "./configuration";

const levels = {
    alert: 0,
    error: 1,
    warn: 2,
    http: 3,
    info: 4,
    debug: 5
};

const colors = {
    alert: "red",
    error: "red",
    warn: "magenta",
    http: "blue",
    info: "green",
    debug: "cyan"
}

export default function logger(label: string) {
    winston.addColors(colors);
    return winston.createLogger({
        level: configuration.logLevel,
        levels: levels,
        format: winston.format.combine(
            winston.format.label({ label: label }),
            winston.format.colorize(),
            winston.format.timestamp(),
            winston.format.prettyPrint(),
            winston.format.splat(),
            winston.format.simple()
        ),
        transports: new winston.transports.Console()
    });
}
