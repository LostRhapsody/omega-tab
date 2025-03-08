// these are the snake_case variants that serde will use to serialize the enum
export type CancellationReason =
  | "customer_service"
  | "low_quality"
  | "missing_features"
  | "other"
  | "switched_service"
  | "too_complex"
  | "too_expensive"
  | "unused"
  | "";
