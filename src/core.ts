import axios from "axios";
import { ProblemList } from "./problem";
import { GroupStatus } from "./group";

const API_BASE_URL = "http://localhost:8000/api";

export async function getProblemList(): Promise<ProblemList> {

        const response = await axios.get(`${API_BASE_URL}/problems`);
        return response.data;
    
}

export async function getAllProblems(): Promise<ProblemList> {
    const response = await axios.get(`${API_BASE_URL}/problems/all`);
    return response.data;
}

export async function getProblemAnswer(id: number): Promise<string> {
    const response = await axios.get(`${API_BASE_URL}/problem/${id}/answer`);
    return response.data;
}

export async function startNewGame(): Promise<void> {
    await axios.post(`${API_BASE_URL}/start`);
    return;
}

export async function nextProblem(group: number): Promise<void> {
    await axios.get(`${API_BASE_URL}/group/${group}/next`);
    return;
}

export async function getGroupStatus(): Promise<GroupStatus[]> {
    const response = await axios.get(`${API_BASE_URL}/group/status`);
    return response.data;
}

// export async function judge(group: number, correct: boolean): Promise<void> {
//     await axios.post(`${API_BASE_URL}/group/${group}/${correct ? "correct" : "wrong"}`);
//     return;
// }