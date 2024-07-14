# rivet-head-diary workspace - just command runner configuration

dev-site:
  cd site && trunk serve --proxy-backend=http://localhost:8000/api

build-site:
  cd site && trunk build --release

run-local:
  cargo shuttle run

deploy-dirty:
  cargo shuttle project restart && cargo shuttle deploy --allow-dirty

deploy:
  cargo shuttle project restart && cargo shuttle deploy --no-test