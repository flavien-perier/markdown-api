import { Response } from "express";

export default abstract class HttpError extends Error {
    constructor(
        protected _statusMessage: string
    ) {
        super(_statusMessage);
    }

    protected abstract get statusCode(): number;

    public apply(res: Response) {
        res.statusMessage = this._statusMessage;
        res.status(this.statusCode).end();
    }
}
