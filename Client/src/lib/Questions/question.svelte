<script lang="ts">
	import { RadioGroup, RadioGroupItem } from '$lib/components/ui/radio-group';
    import { Label } from '$lib/components/ui/label';
    import { questions } from '$lib/types';
	import { selected_answers } from '$lib/stores';
    export let question_number = 0;
    export let question = questions[0];
    export let selected_answer: string = "";

    let answers = question.answers.sort(() => Math.random() - 0.5);

 $: {

    if (selected_answer) {
        selected_answers.update((old) => {
            old[question_number] = answers[parseInt(selected_answer)];
            return old;
        }) ;
    }
 }
</script>


<div class="space-y-4">
    <p class="text-xl font-semibold">{question.question}</p>
    <RadioGroup bind:value={selected_answer}>
        {#each answers as answer, i}
            <div class="flex items-center space-x-2">
                <RadioGroupItem value={i.toString()} id={"answer-" + question_number + "-" + i} name={question.question} />
                <Label for={"answer-" + question_number + "-" + i} class="text-sm">{answer}</Label>
            </div>
        {/each}
    </RadioGroup>
</div>
