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






export const defaultArticle: Article = {
    title: 'Default Title',
    description: 'Default Description',
    body: '<p>Default Body</p>',
    readingTime: 10,
    createdAt: '2024-02-22T16:40:18.000Z',
    publishedAt: '2024-02-29T16:40:18.000Z',
    updatedAt: "",
    lastUpdatedAt: "",
    image: {
        url: 'https://placehold.co/600x400',
        alt: 'Default Image',
        caption: 'Default Caption'
    },
    authors: [
        {
            name: 'Default Author',
            authorBio: 'Default Author Bio',
            slug: 'default-author'
        }
    ],
    questions: [
        {
            question: 'Default Question',
            answers: ['Default Answer 1', 'Default Answer 2', 'Default Answer 3'],
            correct_answer: 'Default Answer'
        }
    ],
    professor: {
        name: 'Default Professor',
        professorBio: 'Default Professor Bio',
        slug: 'default-professor'
    }
};
