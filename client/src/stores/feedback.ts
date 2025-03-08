import { defineStore } from "pinia";

type FeedbackState = {
  reasons: string | null;
  feedbackComment: string | null;
};

export const useFeedbackStore = defineStore("feedbackStore", {
  state: (): FeedbackState => ({
    reasons: null,
    feedbackComment: null,
  }),
  actions: {
    addReason(reason: string) {
      if (reason.length === 0) {
        this.reasons = null;
        return;
      }
      this.reasons = reason;
    },
    setFeedbackComment(comment: string) {
      if (comment.length === 0) {
        this.feedbackComment = null;
        return;
      }
      this.feedbackComment = comment;
    },
    storeFeedback(reasons: string, comment: string) {
      if (reasons.length === 0 && comment.length !== 0) {        
        this.reasons = null;
        this.feedbackComment = comment;
        return;
      }
      if (comment.length === 0 && reasons.length !== 0) {
        this.feedbackComment = null;
        this.reasons = reasons;
        return;
      }
      if(reasons.length === 0 && comment.length === 0) {
        this.reasons = null;
        this.feedbackComment = null;
        return;
      }
      this.reasons = reasons;
      this.feedbackComment = comment;
    },
    clearFeedback() {
      this.reasons = "";
      this.feedbackComment = "";
    },
  },
});
