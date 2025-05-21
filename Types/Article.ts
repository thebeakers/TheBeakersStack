export interface Image {
	url: string;
	alt: string;
	caption: string;
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

export interface Question {
	question: string;
	answers: string[];
	correct_answer: string;
}

export interface Article {
	title: string;
	description: string;
	body: string;
	category: string;
	image: Image;
	authors: Author[];
	professor: Professor;
	questions: Question[]; // Will be Some(Vec<Question>) or Some([]) in Rust
	createdAt: string; // ISO string
	publishedAt: string; // ISO string
	readingTime: number;
	updatedAt: string | null; // ISO string or null
	lastUpdatedAt: string | null; // ISO string or null
}

export const defaultArticle: Article = {
	title: 'Default Article Title',
	description: 'A default description for the article.',
	body: '<p>This is the default <strong>HTML body</strong> content.</p><h2>Subtitle</h2><p>More content here.</p>',
	category: 'General',
	image: {
		url: 'https://placehold.co/600x400/EEE/31343C?text=Default+Image',
		alt: 'Default placeholder image',
		caption: 'This is a default image caption.'
	},
	authors: [
		{
			name: 'Dr. Default Author',
			authorBio: 'The default author of this article.',
			slug: 'default-author'
		}
	],
	professor: {
		name: 'Professor Default',
		professorBio: 'The default professor reviewing this article.',
		slug: 'default-professor'
	},
	questions: [
		{
			question: 'What is the default question?',
			answers: ['Answer A', 'Answer B (Correct)', 'Answer C', 'Answer D'],
			correct_answer: 'Answer B (Correct)'
		},
		{
			question: 'Is this another default question?',
			answers: ['Yes (Correct)', 'No', 'Maybe'],
			correct_answer: 'Yes (Correct)'
		}
	],
	createdAt: new Date().toISOString(),
	publishedAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(), // 7 days from now
	readingTime: 5,
	updatedAt: null,
	lastUpdatedAt: null
};