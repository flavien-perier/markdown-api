import HttpError from "./http.error";

export default class HttpInternalServerError extends HttpError {
    protected get statusCode() {
        return 500;
    }
}
