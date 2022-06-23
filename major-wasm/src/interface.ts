export interface HoleKindText {
    type: 'text';
}

export interface HoleKindImage {
    type: 'image';
    url: string;
}

export interface HoleKindAudio {
    type: 'audio';
    url: string;
}

export type HoleKind = HoleKindText | HoleKindImage | HoleKindAudio

export interface Hole {
    id: number;
    text: string;
    kind: HoleKind;
    timestamp: string;
    reply: number;
    likenum: number;
    tag: string | null;
}

export interface HoleEntry {
    entry: Hole;
    snapshot: string;
}

export interface Reply {
    id: number;
    hole: number;
    name: string;
    text: string;
    dz: boolean;
    timestamp: string;
    tag: string | null;
}

export interface ReplyEntry {
    entry: Reply;
    snapshot: string;
}
