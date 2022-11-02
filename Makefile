test : core serve

core : build copy download

build : 
	cargo build --release --target wasm32-unknown-unknown

copy :
	cp ./target/wasm32-unknown-unknown/release/*.wasm ./web/game.wasm

download :
	wget -nc https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js -P ./web/ \
	wget -nc https://raw.githubusercontent.com/optozorax/quad-url/master/js/quad-url.js -P ./web/ \
	wget -nc https://raw.githubusercontent.com/not-fl3/sapp-jsutils/master/js/sapp_jsutils.js -P ./web/ \

serve :
	basic-http-server ./web
