BASE_DOMAIN ?= biyard.co
DOMAIN ?= $(PROJECT).$(ENV).$(BASE_DOMAIN)

ENV ?= local
PROJECT ?= $(shell basename `git rev-parse --show-toplevel`)
SERVICE ?= web
COMMIT ?= $(shell git rev-parse --short HEAD)

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
CDN_ID ?= $(shell aws cloudfront list-distributions --query "DistributionList.Items[*].{id:Id,test:AliasICPRecordals[?CNAME=='$(DOMAIN)']}" --output json |jq '. | map(select(.test | length > 0))[0] | .id' | tr -d \")
ACM_ID ?= $(shell aws acm list-certificates --query "CertificateSummaryList[*].{id:CertificateArn,domains:SubjectAlternativeNameSummaries}[?contains(domains,'$(DOMAIN)')].id" --output text --region us-east-1)
HOSTED_ZONE_ID ?= $(shell basename `aws route53 list-hosted-zones-by-name --dns-name $(BASE_DOMAIN) --query "HostedZones[0].Id" --output text`)
WORKSPACE_ROOT ?= $(PWD)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
VPC_ID ?= $(shell aws ec2 describe-vpcs --query "Vpcs[0].VpcId" --output json | tr -d \")
TABLE_NAME ?= $(PROJECT)-$(ENV)

ENABLE_S3 ?= false
ENABLE_DYNAMO ?= false
ENABLE_FARGATE ?= false
ENABLE_LAMBDA ?= true
ENABLE_OPENSEARCH ?= false

BUILD_CDK_ENV ?= AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) DOMAIN=$(DOMAIN) TABLE_NAME=$(TABLE_NAME) CDN_ID=$(CDN_ID) ACM_ID=$(ACM_ID) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID) WORKSPACE_ROOT=$(WORKSPACE_ROOT) SERVICE=$(SERVICE) VPC_ID=$(VPC_ID) AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) COMMIT=$(COMMIT) ENV=$(ENV) ENABLE_S3=$(ENABLE_S3) ENABLE_DYNAMO=$(ENABLE_DYNAMO) ENABLE_FARGATE=$(ENABLE_FARGATE) ENABLE_LAMBDA=$(ENABLE_LAMBDA) ENABLE_OPENSEARCH=$(ENABLE_OPENSEARCH)

run:
	cd packages/$(SERVICE) && make run

.PHONY: build
build:
	mkdir -p .build/$(SERVICE)
	cd packages/$(SERVICE) && ENV=$(ENV) ARTIFACT_DIR=$(PWD)/.build/$(SERVICE) make build
