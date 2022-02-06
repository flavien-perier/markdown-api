import HttpError from "./http.error";

export default class HttpForbidenError extends HttpError {
    protected get statusCode() {
        return 403;
    }
}
