export type SampleDocument = {
  id: string;
  label: string;
  source: string;
};

export const sampleDocuments: SampleDocument[] = [
  {
    id: "mixed",
    label: "기본",
    source: `# MarkHangeul 샘플

오늘은 날씨가 좋네요{↘}.
안녕{↗—!}하세요.
Hello{!↗} world{—}.
妈{T1} 麻{T2} 马{T3} 骂{T4}
((want to)){reduced=true,stress=weak,duration=short}
녕{pitch=rise,duration=long,stress=strong}`,
  },
  {
    id: "tone",
    label: "성조",
    source: `妈{T1} 麻{T2} 马{T3} 骂{T4}
ma{tone=1} ma{tone=2} ma{tone=3} ma{tone=4}
こ{↑} え{↓} か{↗} き{↘}`,
  },
  {
    id: "range",
    label: "범위",
    source: `((정말입니까)){pitch=rise,stress=strong}
((good morning)){pitch=fall,duration=long}
((want to)){reduced=true,stress=weak,duration=short,note=casual speech}`,
  },
  {
    id: "errors",
    label: "오류",
    source: `안녕{↗—!}
Hello{pitch=curve}
妈{T7}
((닫히지 않은 범위){pitch=rise}
빈{}표기`,
  },
];
