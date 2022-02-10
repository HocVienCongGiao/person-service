env=dev
bounded_context=biography
entity_name=person
gh_pat=`cat ~/.peterbean/.gh_pat`
organisation=HocVienCongGiao
contract_repo=hvcg-contracts

clean:
	rm -fr ./contracts
	rm -fr *-openapi

download-contracts: clean
	mkdir -p ./contracts/$(bounded_context)/openapi
	curl -s \
    -H "Authorization: token $(gh_pat)" \
  -X GET \
  -H "Accept: application/vnd.github.v3+json" \
  https://api.github.com/repos/$(organisation)/$(contract_repo)/contents/contracts/$(bounded_context)/openapi/$(entity_name).yaml | jq ".download_url" --raw-output | xargs curl -s -X GET > ./contracts/${bounded_context}/openapi/${entity_name}.yaml

unpack:	download-contracts
	echo "Unpacking ... [$(env)]"
	cp -R ./bin/custom_templates/* ./bin/rust-server/
	mkdir -p $(entity_name)-openapi
	java -jar ./bin/openapi-generator-cli.jar generate -i ./contracts/$(bounded_context)/openapi/$(entity_name).yaml -o $(entity_name)-openapi --package-name=hvcg_$(bounded_context)_openapi_$(entity_name) -g rust-server -t ./bin/rust-server -c ./bin/config.yaml --type-mappings=date=NaiveDate
	rm -fr $(entity_name)-openapi/examples
	rm -fr $(entity_name)-openapi/src/client
	rm -fr $(entity_name)-openapi/src/server
	rm $(entity_name)-openapi/src/context.rs
	rm $(entity_name)-openapi/src/header.rs
	cd $(entity_name)-openapi &&  cargo fmt --all