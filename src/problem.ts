export type ProblemStatus = 'unrevealed' | 'answering' | 'solved' | 'failed' | 'judging';

export type Score = 100 | 200 | 300 | 400;

export interface Problem {
    id: number;
    question: string;
    answer: string;
    score: Score;
}

export type ProblemList = {
    status: ProblemStatus;
    score: Score;
    group: number;
    problem?: Problem;
}[];