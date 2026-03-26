from fastapi import FastAPI

app = FastAPI(title="{{project_name}}")


@app.get("/health")
def health():
    return {"status": "ok", "name": "{{project_name}}"}
