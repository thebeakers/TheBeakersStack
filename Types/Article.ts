export interface Article {
    title: string;
    description: string;
    body: string;
    image: {
        url: string;
        alt: string;
        caption: string;
    };
    authors: {
        name: string;
        authorBio: string;
        slug: string;
    }[];
    professor: {
        name: string;
        professorBio: string;
        slug: string;
    };
    questions: Question[];
    createdAt: string;
    publishedAt: string;
    readingTime: number;
    updatedAt: null | string;
    lastUpdatedAt: null | string;
}

export type Question = {
    question: string;
    answers: string[];
    correct_answer: string;
}
export let questions: Question[] = [{ 'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct' },
{ 'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct' },
{ 'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct' },
{ 'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct' },
{ 'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct' },
{ 'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct' },
{ 'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct' },
{ 'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct' },
{ 'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct' },
{ 'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct' }];

