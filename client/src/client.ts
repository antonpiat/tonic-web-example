import {GrpcWebFetchTransport} from "@protobuf-ts/grpcweb-transport";
import {GrpcWebServiceClient} from "./proto/grpc-web.client.ts";

const transport = () => new GrpcWebFetchTransport({
    baseUrl: "http://127.0.0.1:50051"
})

export class GrpcWebService extends GrpcWebServiceClient {
    constructor() {
        super(transport());
    }
}