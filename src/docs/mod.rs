use axum::{Router, response::Html, http::StatusCode, Json};
use axum::routing::get;
use serde_json::json;

pub async fn swagger_ui() -> Html<&'static str> {
    Html(include_str!("swagger_ui.html"))
}

pub async fn openapi_json() -> Json<serde_json::Value> {
    Json(json!({
        "openapi": "3.0.3",
        "info": {
            "title": "Philand Budget Tracking API",
            "description": "A comprehensive budget tracking and financial management API built with Rust and Axum",
            "version": "1.0.0",
            "contact": {
                "name": "Philand Team",
                "url": "https://phila.cloud"
            }
        },
        "servers": [
            {
                "url": "http://localhost:8080",
                "description": "Development server"
            },
            {
                "url": "https://api.phila.cloud",
                "description": "Production server"
            }
        ],
        "paths": {
            "/healthz": {
                "get": {
                    "tags": ["Health"],
                    "summary": "Health check",
                    "description": "Check if the API is running",
                    "responses": {
                        "200": {
                            "description": "API is healthy",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "status": {
                                                "type": "string",
                                                "example": "ok"
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        "500": {
                            "description": "Internal server error",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Error"}
                                }
                            }
                        }
                    }
                }
            },
            "/auth/signup": {
                "post": {
                    "tags": ["Authentication"],
                    "summary": "User registration",
                    "description": "Create a new user account",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["email", "password"],
                                    "properties": {
                                        "email": {
                                            "type": "string",
                                            "format": "email"
                                        },
                                        "password": {
                                            "type": "string",
                                            "minLength": 8
                                        },
                                        "name": {
                                            "type": "string"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "User created successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "token": {"type": "string"},
                                            "user": {"$ref": "#/components/schemas/User"}
                                        }
                                    }
                                }
                            }
                        },
                        "400": {
                            "description": "Invalid input",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Error"}
                                }
                            }
                        },
                        "409": {
                            "description": "Email already exists",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Error"}
                                }
                            }
                        }
                    }
                }
            },
            "/auth/login": {
                "post": {
                    "tags": ["Authentication"],
                    "summary": "User login",
                    "description": "Authenticate user and get JWT token",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["email", "password"],
                                    "properties": {
                                        "email": {
                                            "type": "string",
                                            "format": "email"
                                        },
                                        "password": {
                                            "type": "string"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Login successful",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "token": {
                                                "type": "string"
                                            },
                                            "user": {
                                                "$ref": "#/components/schemas/User"
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Invalid credentials",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Error"}
                                }
                            }
                        },
                        "400": {
                            "description": "Invalid input",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Error"}
                                }
                            }
                        }
                    }
                }
            },
            "/auth/logout": {
                "post": {
                    "tags": ["Authentication"],
                    "summary": "User logout",
                    "description": "Logout user (invalidate token)",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "Logout successful"
                        }
                    }
                }
            },
            "/api/profile": {
                "get": {
                    "tags": ["Profile"],
                    "summary": "Get user profile",
                    "description": "Get current user's profile information",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "User profile",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/User"
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Unauthorized"
                        }
                    }
                },
                "patch": {
                    "tags": ["Profile"],
                    "summary": "Update user profile",
                    "description": "Update current user's profile information",
                    "security": [{"bearerAuth": []}],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string"},
                                        "bio": {"type": "string"},
                                        "timezone": {"type": "string"},
                                        "locale": {"type": "string"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Profile updated successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/User"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/budgets": {
                "get": {
                    "tags": ["Budgets"],
                    "summary": "List budgets",
                    "description": "Get all budgets accessible to the current user",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "query",
                            "in": "query",
                            "description": "Search query for budget names",
                            "schema": {"type": "string"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "List of budgets with user roles",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {"$ref": "#/components/schemas/Budget"}
                                    }
                                }
                            }
                        }
                    }
                },
                "post": {
                    "tags": ["Budgets"],
                    "summary": "Create budget",
                    "description": "Create a new budget",
                    "security": [{"bearerAuth": []}],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["name", "currency_code", "budget_type", "period_type"],
                                    "properties": {
                                        "name": {"type": "string"},
                                        "description": {"type": "string"},
                                        "currency_code": {"type": "string", "example": "USD"},
                                        "budget_type": {
                                            "type": "string",
                                            "enum": ["standard", "saving", "debt", "invest", "sharing"]
                                        },
                                        "period_type": {"type": "string", "example": "monthly"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Budget created successfully",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Budget"}
                                }
                            }
                        }
                    }
                }
            },
            "/api/budgets/{id}": {
                "get": {
                    "tags": ["Budgets"],
                    "summary": "Get budget",
                    "description": "Get budget details by ID",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Budget details",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Budget"}
                                }
                            }
                        },
                        "404": {
                            "description": "Budget not found"
                        }
                    }
                },
                "patch": {
                    "tags": ["Budgets"],
                    "summary": "Update budget",
                    "description": "Update budget information",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string"},
                                        "description": {"type": "string"},
                                        "currency_code": {"type": "string"},
                                        "budget_type": {
                                            "type": "string",
                                            "enum": ["standard", "saving", "debt", "invest", "sharing"]
                                        },
                                        "archived": {"type": "boolean"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Budget updated successfully",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Budget"}
                                }
                            }
                        }
                    }
                },
                "delete": {
                    "tags": ["Budgets"],
                    "summary": "Delete budget",
                    "description": "Delete a budget permanently",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Budget deleted successfully"
                        }
                    }
                }
            },
            "/api/budgets/{id}/balance": {
                "get": {
                    "tags": ["Budgets"],
                    "summary": "Get budget balance",
                    "description": "Get current balance (income, expense, net) for a budget",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Budget balance",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "income": {"type": "integer", "description": "Total income in minor currency units"},
                                            "expense": {"type": "integer", "description": "Total expense in minor currency units"},
                                            "net": {"type": "integer", "description": "Net balance (income - expense) in minor currency units"},
                                            "currency_code": {"type": "string"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/budgets/{id}/categories": {
                "get": {
                    "tags": ["Categories"],
                    "summary": "List categories",
                    "description": "Get all categories for a budget",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "List of categories",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {"$ref": "#/components/schemas/Category"}
                                    }
                                }
                            }
                        }
                    }
                },
                "post": {
                    "tags": ["Categories"],
                    "summary": "Create category",
                    "description": "Create a new category for a budget",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["name", "kind"],
                                    "properties": {
                                        "name": {"type": "string"},
                                        "kind": {"type": "string", "enum": ["income", "expense"]},
                                        "icon": {"type": "string"},
                                        "color": {"type": "string"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Category created successfully",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Category"}
                                }
                            }
                        }
                    }
                }
            },
            "/api/budgets/{id}/entries": {
                "get": {
                    "tags": ["Entries"],
                    "summary": "List entries",
                    "description": "Get all entries for a budget with filtering options",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "kind",
                            "in": "query",
                            "description": "Filter by entry type",
                            "schema": {"type": "string", "enum": ["income", "expense"]}
                        },
                        {
                            "name": "category_id",
                            "in": "query",
                            "description": "Filter by category ID",
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "from",
                            "in": "query",
                            "description": "Start date filter (YYYY-MM-DD)",
                            "schema": {"type": "string", "format": "date"}
                        },
                        {
                            "name": "to",
                            "in": "query",
                            "description": "End date filter (YYYY-MM-DD)",
                            "schema": {"type": "string", "format": "date"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "List of entries",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {"$ref": "#/components/schemas/Entry"}
                                    }
                                }
                            }
                        }
                    }
                },
                "post": {
                    "tags": ["Entries"],
                    "summary": "Create entry",
                    "description": "Create a new entry for a budget",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["amount_minor", "entry_date", "kind", "category_id"],
                                    "properties": {
                                        "amount_minor": {
                                            "type": "integer",
                                            "description": "Amount in minor currency units (cents)",
                                            "example": 500000
                                        },
                                        "entry_date": {"type": "string", "format": "date"},
                                        "kind": {"type": "string", "enum": ["income", "expense"]},
                                        "description": {"type": "string"},
                                        "counterparty": {"type": "string"},
                                        "category_id": {"type": "string", "format": "uuid"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Entry created successfully",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Entry"}
                                }
                            }
                        }
                    }
                }
            },
            "/api/budgets/{id}/entries/{entry_id}": {
                "patch": {
                    "tags": ["Entries"],
                    "summary": "Update entry",
                    "description": "Update an existing entry",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "entry_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "amount_minor": {"type": "integer"},
                                        "entry_date": {"type": "string", "format": "date"},
                                        "kind": {"type": "string", "enum": ["income", "expense"]},
                                        "description": {"type": "string"},
                                        "counterparty": {"type": "string"},
                                        "category_id": {"type": "string", "format": "uuid"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Entry updated successfully",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Entry"}
                                }
                            }
                        }
                    }
                },
                "delete": {
                    "tags": ["Entries"],
                    "summary": "Delete entry",
                    "description": "Delete an entry",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "entry_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Entry deleted successfully"
                        }
                    }
                }
            },
            "/api/budgets/{id}/members": {
                "get": {
                    "tags": ["Members"],
                    "summary": "List members",
                    "description": "Get all members of a budget",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "List of budget members",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {"$ref": "#/components/schemas/Member"}
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/notifications": {
                "get": {
                    "tags": ["Notifications"],
                    "summary": "List notifications",
                    "description": "Get all notifications for the current user",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "List of notifications",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {"$ref": "#/components/schemas/Notification"}
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/notifications/unread-count": {
                "get": {
                    "tags": ["Notifications"],
                    "summary": "Get unread count",
                    "description": "Get the count of unread notifications",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "Unread notification count",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "count": {"type": "integer"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/notifications/mark-read": {
                "post": {
                    "tags": ["Notifications"],
                    "summary": "Mark notifications as read",
                    "description": "Mark specific notifications as read",
                    "security": [{"bearerAuth": []}],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["notification_ids"],
                                    "properties": {
                                        "notification_ids": {
                                            "type": "array",
                                            "items": {"type": "string", "format": "uuid"}
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Notifications marked as read successfully"
                        }
                    }
                }
            },
            "/api/notifications/mark-all-read": {
                "post": {
                    "tags": ["Notifications"],
                    "summary": "Mark all notifications as read",
                    "description": "Mark all notifications as read for the current user",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "All notifications marked as read successfully"
                        }
                    }
                }
            },
            "/api/users": {
                "get": {
                    "tags": ["Users"],
                    "summary": "List users",
                    "description": "Get list of all users (admin only)",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "List of users",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {"$ref": "#/components/schemas/User"}
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/profile/avatar": {
                "post": {
                    "tags": ["Profile"],
                    "summary": "Upload avatar",
                    "description": "Upload user avatar image",
                    "security": [{"bearerAuth": []}],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "multipart/form-data": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "avatar": {
                                            "type": "string",
                                            "format": "binary"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Avatar uploaded successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "avatar_url": {"type": "string"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "delete": {
                    "tags": ["Profile"],
                    "summary": "Delete avatar",
                    "description": "Delete user avatar image",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "Avatar deleted successfully"
                        }
                    }
                }
            },
            "/api/budgets/{id}/categories/{category_id}": {
                "get": {
                    "tags": ["Categories"],
                    "summary": "Get category",
                    "description": "Get category details by ID",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "category_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Category details",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Category"}
                                }
                            }
                        }
                    }
                },
                "patch": {
                    "tags": ["Categories"],
                    "summary": "Update category",
                    "description": "Update category information",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "category_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string"},
                                        "icon": {"type": "string"},
                                        "color": {"type": "string"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Category updated successfully",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Category"}
                                }
                            }
                        }
                    }
                },
                "delete": {
                    "tags": ["Categories"],
                    "summary": "Delete category",
                    "description": "Delete a category",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "category_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Category deleted successfully"
                        }
                    }
                }
            },
            "/api/budgets/{id}/entries/{entry_id}/comments": {
                "get": {
                    "tags": ["Comments"],
                    "summary": "List comments",
                    "description": "Get all comments for an entry",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "entry_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "List of comments",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {"$ref": "#/components/schemas/Comment"}
                                    }
                                }
                            }
                        }
                    }
                },
                "post": {
                    "tags": ["Comments"],
                    "summary": "Create comment",
                    "description": "Create a new comment for an entry",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "entry_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["content"],
                                    "properties": {
                                        "content": {"type": "string"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Comment created successfully",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Comment"}
                                }
                            }
                        }
                    }
                }
            },
            "/api/budgets/{id}/entries/{entry_id}/comments/{comment_id}": {
                "patch": {
                    "tags": ["Comments"],
                    "summary": "Update comment",
                    "description": "Update an existing comment",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "entry_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "comment_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "content": {"type": "string"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Comment updated successfully",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Comment"}
                                }
                            }
                        }
                    }
                },
                "delete": {
                    "tags": ["Comments"],
                    "summary": "Delete comment",
                    "description": "Delete a comment",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "entry_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "comment_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Comment deleted successfully"
                        }
                    }
                }
            },
            "/api/budgets/{id}/entries/{entry_id}/attachments": {
                "post": {
                    "tags": ["Attachments"],
                    "summary": "Upload attachment",
                    "description": "Upload file attachment for an entry",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "entry_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "multipart/form-data": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "file": {
                                            "type": "string",
                                            "format": "binary"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Attachment uploaded successfully",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/Attachment"}
                                }
                            }
                        }
                    }
                }
            },
            "/api/budgets/{id}/entries/{entry_id}/attachments/{attachment_id}": {
                "delete": {
                    "tags": ["Attachments"],
                    "summary": "Delete attachment",
                    "description": "Delete an attachment",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "entry_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "attachment_id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Attachment deleted successfully"
                        }
                    }
                }
            },
            "/api/budgets/{id}/summary/monthly": {
                "get": {
                    "tags": ["Summaries"],
                    "summary": "Monthly summary",
                    "description": "Get monthly summary data for a budget",
                    "security": [{"bearerAuth": []}],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": {"type": "string", "format": "uuid"}
                        },
                        {
                            "name": "year",
                            "in": "query",
                            "description": "Year for summary (default: current year)",
                            "schema": {"type": "integer"}
                        },
                        {
                            "name": "month",
                            "in": "query",
                            "description": "Month for summary (default: current month)",
                            "schema": {"type": "integer", "minimum": 1, "maximum": 12}
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Monthly summary data",
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/MonthlySummary"}
                                }
                            }
                        }
                    }
                }
            },
            "/api/admin/cleanup": {
                "post": {
                    "tags": ["Admin"],
                    "summary": "Manual cleanup",
                    "description": "Manually trigger cleanup of soft-deleted records (admin only)",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "Cleanup completed successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "message": {"type": "string"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/auth/google": {
                "post": {
                    "tags": ["Authentication"],
                    "summary": "Google OAuth login",
                    "description": "Authenticate user with Google OAuth",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["token"],
                                    "properties": {
                                        "token": {"type": "string", "description": "Google OAuth token"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Login successful",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "token": {"type": "string"},
                                            "user": {"$ref": "#/components/schemas/User"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/auth/forgot/email": {
                "post": {
                    "tags": ["Authentication"],
                    "summary": "Request password reset",
                    "description": "Send password reset OTP to email",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["email"],
                                    "properties": {
                                        "email": {"type": "string", "format": "email"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "OTP sent successfully"
                        }
                    }
                }
            },
            "/auth/forgot/otp": {
                "post": {
                    "tags": ["Authentication"],
                    "summary": "Verify reset OTP",
                    "description": "Verify password reset OTP",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["email", "otp"],
                                    "properties": {
                                        "email": {"type": "string", "format": "email"},
                                        "otp": {"type": "string"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "OTP verified successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "reset_token": {"type": "string"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/auth/reset": {
                "post": {
                    "tags": ["Authentication"],
                    "summary": "Reset password",
                    "description": "Reset password with reset token",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["reset_token", "new_password"],
                                    "properties": {
                                        "reset_token": {"type": "string"},
                                        "new_password": {"type": "string", "minLength": 8}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Password reset successfully"
                        }
                    }
                }
            },
            "/api/transfers": {
                "post": {
                    "tags": ["Transfers"],
                    "summary": "Create transfer",
                    "description": "Transfer money between budgets",
                    "security": [{"bearerAuth": []}],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["from_budget_id", "to_budget_id", "from_category_id", "to_category_id", "amount_minor"],
                                    "properties": {
                                        "from_budget_id": {"type": "string", "format": "uuid"},
                                        "to_budget_id": {"type": "string", "format": "uuid"},
                                        "from_category_id": {"type": "string", "format": "uuid"},
                                        "to_category_id": {"type": "string", "format": "uuid"},
                                        "amount_minor": {"type": "integer", "description": "Amount in minor currency units"},
                                        "description": {"type": "string"},
                                        "entry_date": {"type": "string", "format": "date"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Transfer completed successfully"
                        }
                    }
                }
            }
        },
        "components": {
            "securitySchemes": {
                "bearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "JWT"
                }
            },
            "schemas": {
                "User": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "format": "uuid"},
                        "email": {"type": "string", "format": "email"},
                        "name": {"type": "string"},
                        "bio": {"type": "string", "nullable": true},
                        "avatar_url": {"type": "string", "nullable": true},
                        "timezone": {"type": "string"},
                        "locale": {"type": "string"},
                        "created_at": {"type": "string", "format": "date-time"},
                        "updated_at": {"type": "string", "format": "date-time"}
                    }
                },
                "Budget": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "format": "uuid"},
                        "name": {"type": "string"},
                        "description": {"type": "string", "nullable": true},
                        "currency_code": {"type": "string", "example": "USD"},
                        "budget_type": {
                            "type": "string",
                            "enum": ["standard", "saving", "debt", "invest", "sharing"]
                        },
                        "period_type": {"type": "string", "example": "monthly"},
                        "archived": {"type": "boolean"},
                        "created_by": {"type": "string", "format": "uuid"},
                        "created_at": {"type": "string", "format": "date-time"},
                        "updated_at": {"type": "string", "format": "date-time"},
                        "user_role": {"type": "string", "enum": ["owner", "admin", "member", "viewer"]}
                    }
                },
                "Category": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "format": "uuid"},
                        "budget_id": {"type": "string", "format": "uuid"},
                        "name": {"type": "string"},
                        "kind": {"type": "string", "enum": ["income", "expense"]},
                        "icon": {"type": "string", "nullable": true},
                        "color": {"type": "string", "nullable": true},
                        "created_at": {"type": "string", "format": "date-time"},
                        "updated_at": {"type": "string", "format": "date-time"}
                    }
                },
                "Entry": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "format": "uuid"},
                        "budget_id": {"type": "string", "format": "uuid"},
                        "category_id": {"type": "string", "format": "uuid"},
                        "amount_minor": {
                            "type": "integer",
                            "description": "Amount in minor currency units (cents)"
                        },
                        "entry_date": {"type": "string", "format": "date"},
                        "kind": {"type": "string", "enum": ["income", "expense"]},
                        "description": {"type": "string", "nullable": true},
                        "counterparty": {"type": "string", "nullable": true},
                        "created_by": {"type": "string", "format": "uuid"},
                        "created_at": {"type": "string", "format": "date-time"},
                        "updated_at": {"type": "string", "format": "date-time"},
                        "category": {"$ref": "#/components/schemas/Category"},
                        "created_by_user": {"$ref": "#/components/schemas/User"}
                    }
                },
                "Comment": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "format": "uuid"},
                        "entry_id": {"type": "string", "format": "uuid"},
                        "content": {"type": "string"},
                        "created_by": {"type": "string", "format": "uuid"},
                        "created_at": {"type": "string", "format": "date-time"},
                        "updated_at": {"type": "string", "format": "date-time"},
                        "created_by_user": {"$ref": "#/components/schemas/User"}
                    }
                },
                "Attachment": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "format": "uuid"},
                        "entry_id": {"type": "string", "format": "uuid"},
                        "filename": {"type": "string"},
                        "file_url": {"type": "string"},
                        "file_size": {"type": "integer"},
                        "mime_type": {"type": "string"},
                        "uploaded_by": {"type": "string", "format": "uuid"},
                        "created_at": {"type": "string", "format": "date-time"}
                    }
                },
                "Member": {
                    "type": "object",
                    "properties": {
                        "user_id": {"type": "string", "format": "uuid"},
                        "budget_id": {"type": "string", "format": "uuid"},
                        "role": {"type": "string", "enum": ["owner", "admin", "member", "viewer"]},
                        "joined_at": {"type": "string", "format": "date-time"},
                        "user": {"$ref": "#/components/schemas/User"}
                    }
                },
                "Notification": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "format": "uuid"},
                        "user_id": {"type": "string", "format": "uuid"},
                        "title": {"type": "string"},
                        "message": {"type": "string"},
                        "notification_type": {"type": "string"},
                        "related_id": {"type": "string", "format": "uuid", "nullable": true},
                        "is_read": {"type": "boolean"},
                        "created_at": {"type": "string", "format": "date-time"}
                    }
                },
                "MonthlySummary": {
                    "type": "object",
                    "properties": {
                        "year": {"type": "integer"},
                        "month": {"type": "integer"},
                        "total_income": {"type": "integer", "description": "Total income in minor currency units"},
                        "total_expense": {"type": "integer", "description": "Total expense in minor currency units"},
                        "net_amount": {"type": "integer", "description": "Net amount (income - expense) in minor currency units"},
                        "currency_code": {"type": "string"},
                        "categories": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "category_id": {"type": "string", "format": "uuid"},
                                    "category_name": {"type": "string"},
                                    "kind": {"type": "string", "enum": ["income", "expense"]},
                                    "total_amount": {"type": "integer"},
                                    "entry_count": {"type": "integer"}
                                }
                            }
                        },
                        "daily_totals": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "date": {"type": "string", "format": "date"},
                                    "income": {"type": "integer"},
                                    "expense": {"type": "integer"},
                                    "net": {"type": "integer"}
                                }
                            }
                        }
                    }
                },
                "Error": {
                    "type": "object",
                    "properties": {
                        "error": {"type": "string"},
                        "message": {"type": "string"},
                        "details": {"type": "string", "nullable": true}
                    }
                }
            }
        },
        "security": [
            {
                "bearerAuth": []
            }
        ]
    }))
}

pub fn create_swagger_routes() -> Router {
    Router::new()
        .route("/api", get(swagger_ui))
        .route("/api/openapi.json", get(openapi_json))
}