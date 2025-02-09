import {
    type DateValue,
    fromDate,
    getLocalTimeZone,
    parseAbsoluteToLocal
} from '@internationalized/date';
import { type Article } from "./types";
export class ArticleClass {
    title: string = $state("");
    description: string = $state("");
    body: string = $state("");
    image: {
        url: string;
        alt: string;
        caption: string;
    };
    authors: AuthorClass[] = $state([]);
    professor: ProfessorClass = $state(new ProfessorClass("", "", ""));
    questions: QuestionClass[] = $state([]);
    createdAt: DateValue = $state(parseAbsoluteToLocal('2024-02-22T16:40:18.000Z'));
    publishedAt: DateValue = $state(parseAbsoluteToLocal('2024-02-22T16:40:18.000Z'));
    readingTime: number = $state(0);
    updatedAt: null | string = $state("");
    lastUpdatedAt: null | string = $state("");

    constructor(art: Article) {
        this.title = art.title;
        this.description = art.description;
        this.body = art.body;
        this.image = art.image;
        this.authors = art.authors.map(
            (question) =>
                new AuthorClass(
                    question.name,
                    question.authorBio,
                    question.slug
                )
        );
        this.professor = new ProfessorClass(art.professor.name, art.professor.professorBio, art.professor.slug);
        this.questions = art.questions.map(
            (question) =>
                new QuestionClass(
                    question.question,
                    question.answers,
                    question.correct_answer
                )
        );
        this.createdAt = parseAbsoluteToLocal(art.createdAt);
        this.publishedAt = parseAbsoluteToLocal(art.publishedAt);
        this.readingTime = art.readingTime;
        this.updatedAt = art.updatedAt;
        this.lastUpdatedAt = art.lastUpdatedAt;
    }
}
export class QuestionClass {
    question: string;
    answers: string[];
    correct_answer: string;

    constructor(question: string, answers: string[], correct_answer: string) {
        this.question = question;
        this.answers = answers;
        this.correct_answer = correct_answer;
    }
}

export class AuthorClass {
    name: string;
    authorBio: string;
    slug: string;

    constructor(name: string, authorBio: string, slug: string) {
        this.name = name;
        this.authorBio = authorBio;
        this.slug = slug;
    }
}
export class ProfessorClass {
    name: string;
    professorBio: string;
    slug: string;
    constructor(name: string, professorBio: string, slug: string) {
        this.name = name;
        this.professorBio = professorBio;
        this.slug = slug;
    }
}
