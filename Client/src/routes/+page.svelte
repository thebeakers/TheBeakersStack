<script>
	import Question from '$lib/Questions/question.svelte';
	import { questions } from '$lib/types';
    import { selected_answers } from '$lib/stores';
	import { Button } from '$lib/components/ui/button';
    function submit() {
        console.log($selected_answers);
        let correct_answers = 0;
        for (let i = 0; i < questions.length; i++) {
            if ($selected_answers[i] === questions[i].correct_answer) {
                correct_answers++;
            }
        }
        console.log(correct_answers);
        return correct_answers;
    }

</script>

<main class="p-4 overflow-y-auto ">

<div class="p-4 overflow-y-auto space-y-8">
    {#each questions as question, i}
        <Question question={question} question_number={i}   />
    {/each}
    <Button on:click={submit}>Submit</Button>
</div>
</main>



