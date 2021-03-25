# Books API

- Endpoint : `/books`
  - Supported operations : **GET**, **POST**, **PUT**, **DELETE**

## CLI Commands

```
# Register the task definition
aws ecs register-task-definition --cli-input-json file://taskdef.json
```

## CodeBuild and CodeDeploy specifications
* [buildspec.yml](buildspec.yml)
* [taskdef.json](taskdef.json)

## ENV variables needed for the CodeBuild
* **AWS_REGION** : AWS Region
* **ACCOUNT_ID** : AWS Account ID
* **TASK_EXECUTION_ARN** : ECS task execution role
