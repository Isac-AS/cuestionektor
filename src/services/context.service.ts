import { ref } from "vue";
import { Question, Questionnaire } from "../models/questionnaire";
import { getQuestionnaires, touchQuestionnaire } from "./questionnaire.service";
import { createEmptyQuestion, getQuestions } from "./question.service";

export type LoadQuestionnaires = { (): Promise<void> };
export type OpenQuestionnaire = { (questionnaireId: number): void };
export type RemoveQuestion = { (question: Question): Promise<void> };
export type ClearLoadedQuestions = { (): Promise<void> };
export type AddEmptyQuestion = { (): Promise<void> };
export type RefreshQuestions = { (): Promise<void> };

export default function useContext() {
    const registeredQuestionnaires = ref<Questionnaire[]>();
    const loadQuestionnaires: LoadQuestionnaires = async () => {
        let questionnaireResponse = await getQuestionnaires();
        registeredQuestionnaires.value = questionnaireResponse.data.sort(
            (a, b) => (a.last_accessed < b.last_accessed) ? 1 : (b.last_accessed < a.last_accessed) ? -1 : 0
        );
    };

    const loadedQuestions = ref<Question[]>();
    const currentQuestionnaireId = ref();
    const openQuestionnaire: OpenQuestionnaire = async (questionnaireId: number) => {
        currentQuestionnaireId.value = questionnaireId;
        touchQuestionnaire(questionnaireId);
        loadedQuestions.value = (await getQuestions(questionnaireId)).data;
        loadedQuestions.value.sort(
            (a, b) => (a.question_number > b.question_number) ? 1 : (b.question_number > a.question_number) ? -1 : 0
        )
    }

    const removeQuestion: RemoveQuestion = async (question: Question) => {
        loadedQuestions.value = loadedQuestions!.value!.filter(obj => { return obj !== question });
        loadedQuestions.value.sort(
            (a, b) => (a.question_number > b.question_number) ? 1 : (b.question_number > a.question_number) ? -1 : 0
        )
    }

    const addEmptyQuestion: AddEmptyQuestion = async () => {
        createEmptyQuestion(currentQuestionnaireId.value)
    }

    const clearLoadedQuestions: ClearLoadedQuestions = async () => {
        loadedQuestions.value = [];
    }

    const refreshQuestions: RefreshQuestions = async () => {
        touchQuestionnaire(currentQuestionnaireId.value);
        loadedQuestions.value = (await getQuestions(currentQuestionnaireId.value)).data;
        loadedQuestions.value.sort(
            (a, b) => (a.question_number > b.question_number) ? 1 : (b.question_number > a.question_number) ? -1 : 0
        )
    }

    return {
        registeredQuestionnaires,
        loadQuestionnaires,
        currentQuestionnaireId,
        loadedQuestions,
        openQuestionnaire,
        removeQuestion,
        clearLoadedQuestions,
        addEmptyQuestion,
        refreshQuestions
    }
}