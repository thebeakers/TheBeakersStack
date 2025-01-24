export interface Article {
    title: string;
    description: string;
    body: string;
    image: {
        url: string;
        alt: string;
        caption: string;
    };
    authors: Author[];
    professor: Professor;
    questions: Question[];
    createdAt: string;
    publishedAt: string;
    readingTime: number;
    updatedAt: null | string;
    lastUpdatedAt: null | string;
}

export interface Question {
    question: string;
    answers: string[];
    correct_answer: string;
}

export interface Author {
    name: string;
    authorBio: string;
    slug: string;
}
export interface Professor {
    name: string;
    professorBio: string;
    slug: string;

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

