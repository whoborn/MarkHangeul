import type {
  AttributeScalar,
  DurationValue,
  MarkHangeulAttributes,
  ParsedAnnotation,
  ParseError,
  PitchValue,
  StressValue,
  ToneValue,
  VolumeValue,
} from "./types";

const booleanKeys = new Set([
  "nasal",
  "aspiration",
  "fortis",
  "lenis",
  "palatalization",
  "retroflexion",
  "liaison",
  "reduced",
  "assimilation",
  "deletion",
]);

const knownStringKeys = new Set(["ipa", "phoneme", "lang", "note", "mora", "syllableRole"]);

const keyAliases: Record<string, keyof MarkHangeulAttributes> = {
  syllable_role: "syllableRole",
  syllablerole: "syllableRole",
};

const pitchAliases: Record<string, PitchValue> = {
  low: "low",
  down: "low",
  mid: "mid",
  middle: "mid",
  high: "high",
  up: "high",
  rise: "rise",
  rising: "rise",
  fall: "fall",
  falling: "fall",
};

const durationAliases: Record<string, DurationValue> = {
  short: "short",
  brief: "short",
  normal: "normal",
  mid: "normal",
  long: "long",
  "extra-long": "extra-long",
  extra_long: "extra-long",
  extralong: "extra-long",
  verylong: "extra-long",
};

const stressAliases: Record<string, StressValue> = {
  weak: "weak",
  light: "weak",
  normal: "normal",
  strong: "strong",
  stressed: "strong",
  "extra-strong": "extra-strong",
  extra_strong: "extra-strong",
  extrastrong: "extra-strong",
};

const volumeAliases: Record<string, VolumeValue> = {
  soft: "soft",
  quiet: "soft",
  weak: "soft",
  normal: "normal",
  loud: "loud",
  strong: "loud",
};

const toneAliases: Record<string, ToneValue> = {
  "0": "neutral",
  t0: "neutral",
  neutral: "neutral",
  none: "neutral",
  "1": "1",
  t1: "1",
  "2": "2",
  t2: "2",
  "3": "3",
  t3: "3",
  "4": "4",
  t4: "4",
};

export function parseAnnotation(rawAnnotation: string, baseIndex = 0): ParsedAnnotation {
  const attributes: MarkHangeulAttributes = {};
  const errors: ParseError[] = [];
  const raw = rawAnnotation.trim();

  if (!raw) {
    errors.push(makeError("EMPTY_ANNOTATION", "빈 annotation입니다.", baseIndex, rawAnnotation.length || 1));
    return { attributes, errors };
  }

  const parts = raw.split(",").map((part) => part.trim()).filter(Boolean);

  if (parts.some((part) => part.includes("="))) {
    for (const part of parts) {
      const localIndex = rawAnnotation.indexOf(part);
      if (part.includes("=")) {
        parseKeyValuePart(part, attributes, errors, baseIndex + Math.max(localIndex, 0));
      } else {
        parseSymbolPart(part, attributes, errors, baseIndex + Math.max(localIndex, 0));
      }
    }
  } else {
    parseSymbolPart(raw, attributes, errors, baseIndex + rawAnnotation.indexOf(raw));
  }

  return { attributes, errors };
}

function parseSymbolPart(
  part: string,
  attributes: MarkHangeulAttributes,
  errors: ParseError[],
  baseIndex: number,
) {
  let index = 0;

  while (index < part.length) {
    const rest = part.slice(index);
    const current = part[index];

    if (/\s|,/.test(current)) {
      index += 1;
      continue;
    }

    if (/^T[0-4]/i.test(rest)) {
      const value = rest.slice(0, 2).toLowerCase();
      attributes.tone = toneAliases[value];
      index += 2;
      continue;
    }

    if (rest.startsWith("——")) {
      attributes.duration = "extra-long";
      index += 2;
      continue;
    }

    if (rest.startsWith("!!")) {
      attributes.stress = "extra-strong";
      index += 2;
      continue;
    }

    switch (current) {
      case "↗":
        attributes.pitch = "rise";
        break;
      case "↘":
        attributes.pitch = "fall";
        break;
      case "↑":
        attributes.pitch = "high";
        break;
      case "↓":
        attributes.pitch = "low";
        break;
      case "·":
        attributes.pitch = "mid";
        break;
      case "˘":
        attributes.duration = "short";
        break;
      case "-":
        attributes.duration = "normal";
        break;
      case "—":
        attributes.duration = "long";
        break;
      case "!":
        attributes.stress = "strong";
        break;
      case "?":
        attributes.stress = "weak";
        break;
      case "°":
        attributes.volume = "soft";
        break;
      case "•":
        attributes.volume = "normal";
        break;
      case "●":
        attributes.volume = "loud";
        break;
      default:
        errors.push(
          makeError(
            "UNKNOWN_SYMBOL",
            `알 수 없는 annotation 기호 '${current}'입니다.`,
            baseIndex + index,
            current.length,
          ),
        );
        break;
    }

    index += 1;
  }
}

function parseKeyValuePart(
  part: string,
  attributes: MarkHangeulAttributes,
  errors: ParseError[],
  baseIndex: number,
) {
  const separatorIndex = part.indexOf("=");

  if (separatorIndex <= 0) {
    errors.push(makeError("MALFORMED_PAIR", "key=value 형식이 아닙니다.", baseIndex, part.length));
    return;
  }

  const rawKey = part.slice(0, separatorIndex).trim();
  const rawValue = part.slice(separatorIndex + 1).trim();

  if (!rawKey || !rawValue) {
    errors.push(makeError("MALFORMED_PAIR", "key 또는 value가 비어 있습니다.", baseIndex, part.length));
    return;
  }

  const key = normalizeKey(rawKey);
  const value = rawValue.replace(/^["']|["']$/g, "");
  const normalizedValue = value.toLowerCase();

  switch (key) {
    case "pitch": {
      const mapped = pitchAliases[normalizedValue];
      if (mapped) attributes.pitch = mapped;
      else errors.push(invalidValueError(key, value, baseIndex + separatorIndex + 1, rawValue.length));
      return;
    }
    case "duration": {
      const mapped = durationAliases[normalizedValue];
      if (mapped) attributes.duration = mapped;
      else errors.push(invalidValueError(key, value, baseIndex + separatorIndex + 1, rawValue.length));
      return;
    }
    case "stress": {
      const mapped = stressAliases[normalizedValue];
      if (mapped) attributes.stress = mapped;
      else errors.push(invalidValueError(key, value, baseIndex + separatorIndex + 1, rawValue.length));
      return;
    }
    case "volume": {
      const mapped = volumeAliases[normalizedValue];
      if (mapped) attributes.volume = mapped;
      else errors.push(invalidValueError(key, value, baseIndex + separatorIndex + 1, rawValue.length));
      return;
    }
    case "tone": {
      const mapped = toneAliases[normalizedValue];
      if (mapped) attributes.tone = mapped;
      else errors.push(invalidValueError(key, value, baseIndex + separatorIndex + 1, rawValue.length));
      return;
    }
    default:
      break;
  }

  if (booleanKeys.has(key)) {
    attributes[key as keyof MarkHangeulAttributes] = parseBoolean(value) as never;
    return;
  }

  if (knownStringKeys.has(key)) {
    attributes[key as keyof MarkHangeulAttributes] = value as never;
    return;
  }

  attributes.extras = {
    ...attributes.extras,
    [rawKey]: parseScalar(value),
  };
}

function normalizeKey(key: string): string {
  const normalized = key.trim().replace(/-/g, "_");
  return (keyAliases[normalized] as string | undefined) ?? normalized;
}

function parseBoolean(value: string): boolean {
  return /^(true|1|yes|y|on)$/i.test(value);
}

function parseScalar(value: string): AttributeScalar {
  if (/^(true|false)$/i.test(value)) return value.toLowerCase() === "true";
  if (/^-?\d+(\.\d+)?$/.test(value)) return Number(value);
  return value;
}

function invalidValueError(key: string, value: string, index: number, length: number): ParseError {
  return makeError("INVALID_VALUE", `'${key}'에 사용할 수 없는 값 '${value}'입니다.`, index, length);
}

function makeError(code: string, message: string, index: number, length: number): ParseError {
  return {
    code,
    message,
    index,
    length: Math.max(length, 1),
    severity: "error",
  };
}
