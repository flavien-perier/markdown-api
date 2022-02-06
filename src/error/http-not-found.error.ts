import HttpError from "./http.error";

export default class HttpNotFoundError extends HttpError {
    protected get statusCode() {
        return 404;
    }
}
