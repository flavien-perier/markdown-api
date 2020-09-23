export default class BadRequestInformations {
    constructor(
        public message: string,
        public errors: any[]
    ) {}
}
