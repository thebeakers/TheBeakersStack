import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { cubicOut } from "svelte/easing";
import type { TransitionConfig } from "svelte/transition";
import { type Article } from "./types";
import {
	getLocalTimeZone,
	type DateValue
} from '@internationalized/date';

export function return_article_from_backend(article: Article): EditorArticle {
	return {
		title: $state(article.title),
		description: $state(article.description),
		body: $state(article.body),
		image: $state(article.image),
		authors: $state(article.authors),
		professor: $state(article.professor),
		questions: $state(article.questions),
		createdAt: $state(article.createdAt),
		publishedAt: $state(article.publishedAt),
		readingTime: $state(article.readingTime),
		updatedAt: $state(article.updatedAt),
		lastUpdatedAt: $state(article.lastUpdatedAt)
	}
}
export function formatDateForInput(dateValue: DateValue) {
	const date = dateValue.toDate(getLocalTimeZone());
	return new Intl.DateTimeFormat('en-US', {
		year: 'numeric',
		month: 'long',
		day: 'numeric',
		hour: '2-digit',
		minute: '2-digit'
	}).format(date);
}


export function get_article_from_toml(): EditorArticle {
	let article: Article = {
		title: 'Default Title',
		description: 'Default Description',
		body: '<p>Default Body</p>',
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
		professor: {
			name: 'Default Professor',
			professorBio: 'Default Professor Bio',
			slug: 'default-professor'
		},
		questions: [
			{
				question: 'Default Question',
				answers: ['Default Answer 1', 'Default Answer 2', 'Default Answer 3'],
				correct_answer: 'Default Answer'
			}
		],
		createdAt: '2024-02-22T16:40:18.000Z',
		publishedAt: '2024-02-29T16:40:18.000Z',
		readingTime: 10,
		updatedAt: null,
		lastUpdatedAt: null
	}
	return return_article_from_backend(article)
}

export interface EditorArticle {
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














export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

type FlyAndScaleParams = {
	y?: number;
	x?: number;
	start?: number;
	duration?: number;
};

export const flyAndScale = (
	node: Element,
	params: FlyAndScaleParams = { y: -8, x: 0, start: 0.95, duration: 150 }
): TransitionConfig => {
	const style = getComputedStyle(node);
	const transform = style.transform === "none" ? "" : style.transform;

	const scaleConversion = (
		valueA: number,
		scaleA: [number, number],
		scaleB: [number, number]
	) => {
		const [minA, maxA] = scaleA;
		const [minB, maxB] = scaleB;

		const percentage = (valueA - minA) / (maxA - minA);
		const valueB = percentage * (maxB - minB) + minB;

		return valueB;
	};

	const styleToString = (
		style: Record<string, number | string | undefined>
	): string => {
		return Object.keys(style).reduce((str, key) => {
			if (style[key] === undefined) return str;
			return str + `${key}:${style[key]};`;
		}, "");
	};

	return {
		duration: params.duration ?? 200,
		delay: 0,
		css: (t) => {
			const y = scaleConversion(t, [0, 1], [params.y ?? 5, 0]);
			const x = scaleConversion(t, [0, 1], [params.x ?? 0, 0]);
			const scale = scaleConversion(t, [0, 1], [params.start ?? 0.95, 1]);

			return styleToString({
				transform: `${transform} translate3d(${x}px, ${y}px, 0) scale(${scale})`,
				opacity: t
			});
		},
		easing: cubicOut
	};
};