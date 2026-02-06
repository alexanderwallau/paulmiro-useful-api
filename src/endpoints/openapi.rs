use crate::endpoints::ApiResponse;
use serde_json::{json, Value};

#[get("/openapi.json")]
pub fn openapi() -> ApiResponse<Value> {
    let doc = json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Useful API",
            "version": "1.0.0",
            "description": "A small collection of fun and useful endpoints."
        },
        "paths": {
            "/": {
                "get": {
                    "summary": "Hello world",
                    "description": "Returns a friendly greeting in plain text or JSON.",
                    "parameters": [
                        {
                            "name": "format",
                            "in": "query",
                            "required": false,
                            "schema": { "type": "string", "enum": ["json"] },
                            "description": "Set to 'json' to receive a JSON response."
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Successful response",
                            "content": {
                                "text/plain": {
                                    "schema": { "type": "string" }
                                },
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "message": { "type": "string" }
                                        },
                                        "required": ["message"]
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/mensabeer": {
                "get": {
                    "summary": "Mensa beer equivalent",
                    "description": "Returns how many congress beers you get for the price of a Mensa stew.",
                    "parameters": [
                        {
                            "name": "format",
                            "in": "query",
                            "required": false,
                            "schema": { "type": "string", "enum": ["json"] },
                            "description": "Set to 'json' to receive a JSON response."
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Successful response",
                            "content": {
                                "text/plain": {
                                    "schema": { "type": "string" }
                                },
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "beers": { "type": "number" },
                                            "message": { "type": "string" }
                                        },
                                        "required": ["beers", "message"]
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/mensatoshi": {
                "get": {
                    "summary": "Mensa Satoshi price",
                    "description": "Returns the current price of a Mensa stew in Satoshi.",
                    "parameters": [
                        {
                            "name": "format",
                            "in": "query",
                            "required": false,
                            "schema": { "type": "string", "enum": ["json"] },
                            "description": "Set to 'json' to receive a JSON response."
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Successful response",
                            "content": {
                                "text/plain": {
                                    "schema": { "type": "string" }
                                },
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "satoshi": { "type": "number" },
                                            "message": { "type": "string" }
                                        },
                                        "required": ["satoshi", "message"]
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/congressbeer": {
                "get": {
                    "summary": "Congress beer calculator",
                    "description": "Returns how many congress beers you can buy for a given Satoshi amount.",
                    "parameters": [
                        {
                            "name": "satoshi",
                            "in": "query",
                            "required": false,
                            "schema": { "type": "number", "format": "double" },
                            "description": "Amount in Satoshi. Defaults to a preconfigured value."
                        },
                        {
                            "name": "format",
                            "in": "query",
                            "required": false,
                            "schema": { "type": "string", "enum": ["json"] },
                            "description": "Set to 'json' to receive a JSON response."
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Successful response",
                            "content": {
                                "text/plain": {
                                    "schema": { "type": "string" }
                                },
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "congressbeers": { "type": "integer" },
                                            "message": { "type": "string" }
                                        },
                                        "required": ["congressbeers", "message"]
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/shark": {
                "get": {
                    "summary": "IKEA shark availability",
                    "description": "Returns availability of IKEA sharks at a specific store.",
                    "parameters": [
                        {
                            "name": "format",
                            "in": "query",
                            "required": false,
                            "schema": { "type": "string", "enum": ["json"] },
                            "description": "Set to 'json' to receive a JSON response."
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Successful response",
                            "content": {
                                "text/plain": {
                                    "schema": { "type": "string" }
                                },
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "beeghaj": { "type": "integer" },
                                            "smolhaj": { "type": "integer" },
                                            "whale": { "type": "integer" },
                                            "message": { "type": "string" }
                                        },
                                        "required": ["beeghaj", "smolhaj", "whale", "message"]
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/teapot": {
                "get": {
                    "summary": "Teapot status",
                    "description": "Always responds with HTTP 418 I'm a teapot.",
                    "responses": {
                        "418": {
                            "description": "I'm a teapot"
                        }
                    }
                }
            },
            "/openapi.json": {
                "get": {
                    "summary": "OpenAPI document",
                    "description": "Returns this OpenAPI specification in JSON format.",
                    "responses": {
                        "200": {
                            "description": "OpenAPI document",
                            "content": {
                                "application/json": {
                                    "schema": { "type": "object" }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    ApiResponse::Json(doc)
}
