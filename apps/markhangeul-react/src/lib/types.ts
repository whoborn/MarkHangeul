export type MarkHangeulScope = "grapheme" | "word" | "range";

export type PitchValue = "low" | "mid" | "high" | "rise" | "fall";
export type DurationValue = "short" | "normal" | "long" | "extra-long";
export type StressValue = "weak" | "normal" | "strong" | "extra-strong";
export type VolumeValue = "soft" | "normal" | "loud";
export type ToneValue = "1" | "2" | "3" | "4" | "neutral";

export type AttributeScalar = string | number | boolean;

export type MarkHangeulAttributes = {
  pitch?: PitchValue;
  duration?: DurationValue;
  stress?: StressValue;
  volume?: VolumeValue;
  tone?: ToneValue;
  ipa?: string;
  phoneme?: string;
  lang?: string;
  note?: string;
  nasal?: boolean;
  aspiration?: boolean;
  fortis?: boolean;
  lenis?: boolean;
  palatalization?: boolean;
  retroflexion?: boolean;
  liaison?: boolean;
  reduced?: boolean;
  assimilation?: boolean;
  deletion?: boolean;
  mora?: string;
  syllableRole?: string;
  extras?: Record<string, AttributeScalar>;
};

export type TextNode = {
  type: "text";
  text: string;
  start: number;
  end: number;
};

export type MarkHangeulNode = {
  type: "markhangeul";
  id: string;
  text: string;
  rawAnnotation: string;
  scope: MarkHangeulScope;
  attributes: MarkHangeulAttributes;
  start: number;
  end: number;
  annotationStart: number;
  annotationEnd: number;
  errors: ParseError[];
};

export type MarkHangeulAstNode = TextNode | MarkHangeulNode;

export type ParseError = {
  code: string;
  message: string;
  index: number;
  length: number;
  severity: "error" | "warning";
};

export type MarkHangeulDocument = {
  type: "document";
  source: string;
  nodes: MarkHangeulAstNode[];
  errors: ParseError[];
};

export type ParsedAnnotation = {
  attributes: MarkHangeulAttributes;
  errors: ParseError[];
};
