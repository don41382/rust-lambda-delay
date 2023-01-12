# rust-lambda-delay
Simple rust example working as an AWS lambda in api-gateway mode.

I wanted to test lambda functions with rust and understand how to do propper error handling.

## what does this example?
It blocks the incoming request for the given (wait) milliseconds. 
The wait seconds are passed as a query parameter and should be run behind a API-Gateway in proxy mode. 

### build & deploy
Requires a `AWS_PROFILE`with the correct rights for lambda deployment.
```shell
AWS_PROFILE=XXXXX cargo lambda build --release --arm64 
AWS_PROFILE=XXXXX cargo lambda deploy
```

### dev run
See [apigw-request](https://github.com/aws/aws-lambda-go/blob/main/events/testdata/apigw-request.json) for further request parameter.

```shell
cargo lambda watch
cargo lambda invoke --data-ascii '{"queryStringParameters": {"wait": "500"}}'
```