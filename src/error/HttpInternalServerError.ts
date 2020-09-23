import HttpError from "./HttpError";

export default class HttpInternalServerError extends HttpError {
    protected get statusCode() {
        return 500;
    }
}
