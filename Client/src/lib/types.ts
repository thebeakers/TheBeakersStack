
// export let questions: Question[] = [{'question': 'What is the primary focus of this research paper?', 'answers': ['Predicting the synthesizability of inorganic crystal structures using large language models.', 'Developing new methods for synthesizing inorganic compounds.', 'Analyzing the properties of synthesized materials.', 'Exploring new techniques for crystal growth.'], 'correct_answer': 'Predicting the synthesizability of inorganic crystal structures using large language models.'}, {'question': 'What type of machine learning models were used to predict synthesizability?', 'answers': ['Fine-tuned large language models (LLMs) and neural networks.', 'Support vector machines (SVMs).', 'Decision trees.', 'K-nearest neighbors (KNN).'], 'correct_answer': 'Fine-tuned large language models (LLMs) and neural networks.'}, {'question': 'How was structural data converted into a format suitable for LLM input?', 'answers': ['Using Robocrystallographer to generate text-based descriptions.', 'Directly using CIF files.', 'Converting crystal structures into images.', 'Using mathematical equations to represent structures.'], 'correct_answer': 'Using Robocrystallographer to generate text-based descriptions.'}, {'question': 'What is a key advantage of using LLMs for synthesizability prediction, according to the paper?', 'answers': ['The ability to provide human-readable explanations for predictions.', 'The increased accuracy compared to other methods.', 'The lower computational cost.', 'The faster prediction speed.'], 'correct_answer': 'The ability to provide human-readable explanations for predictions.'}, {'question': 'What is the positive-unlabeled learning (PU) approach in this context?', 'answers': ['Treating synthesized materials as positive and not-yet-synthesized as unlabeled data.', 'Treating both synthesized and not-yet-synthesized materials as positive data.', 'Treating synthesized materials as unlabeled and not-yet-synthesized as negative data.', 'Treating all materials as equally labeled data.'], 'correct_answer': 'Treating synthesized materials as positive and not-yet-synthesized as unlabeled data.'}, {'question': 'How does the performance of the fine-tuned LLM compare to the traditional graph-based methods?', 'answers': ['The fine-tuned LLM performs comparably or slightly better.', 'The fine-tuned LLM performs significantly worse.', 'The fine-tuned LLM performs significantly better.', 'There is no significant performance difference.'], 'correct_answer': 'The fine-tuned LLM performs comparably or slightly better.'}, {'question': 'What was the main finding regarding the use of LLM-derived representations?', 'answers': ['LLM-derived representations used as input for a neural network model improve prediction performance.', 'LLM-derived representations perform worse than traditional graph representations.', 'LLM-derived representations show no significant change in performance.', 'LLM-derived representations make no difference to performance.'], 'correct_answer': 'LLM-derived representations used as input for a neural network model improve prediction performance.'}, {'question': 'What did the structural sensitivity tests reveal about the models?', 'answers': ['The models are highly sensitive to even small structural changes.', 'The models are not sensitive to structural changes.', 'The models are only sensitive to large structural changes.', 'The models are equally sensitive to all structural changes.'], 'correct_answer': 'The models are highly sensitive to even small structural changes.'}, {'question': 'How did the LLM-based predictions compare with the thermodynamic stability predictions?', 'answers': ['The LLM-based predictions had better precision and correctly identified non-synthesizable materials.', 'The thermodynamic-based predictions were superior.', 'Both methods performed equally well.', 'Thermodynamic predictions are better at predicting non-synthesizability.'], 'correct_answer': 'The LLM-based predictions had better precision and correctly identified non-synthesizable materials.'}, {'question': 'What does the paper suggest regarding the use of LLM explanations for material design?', 'answers': ['The explanations can guide chemists in modifying or optimizing non-synthesizable structures.', 'The explanations have no impact on material design.', 'The explanations are too complex for practical use.', 'The explanations are not helpful in determining synthesizability.'], 'correct_answer': 'The explanations can guide chemists in modifying or optimizing non-synthesizable structures.'}]

import { parse } from "toml";

export type Question = {
    question: string;
    answers: string[];
    correct_answer: string;
}   

export const deserializeArticleFromToml = (tomlText: string): Article => {
  try {
      const parsedData = parse(tomlText);
      return parsedData as unknown as Article;
  } catch (error) {
      console.error('Error deserializing TOML:', error);
      throw error;
  }
};

export let questions: Question[] = [{'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct'}, 
{'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct'},
{'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct'}, 
{'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct'}, 
{'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct'},
{'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct'},
{'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct'},
{'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct'},
{'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct'},
{'question': 'Select the correct answer', 'answers': ['Correct', 'Incorrect', 'Incorrect', 'Incorrect'], 'correct_answer': 'Correct'}];


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
    createdAt: string;
    publishedAt: string;
    readingTime: number;
    updatedAt: null | string;
    lastUpdatedAt: null | string;
  }