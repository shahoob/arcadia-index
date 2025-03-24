<template>
  <ContentContainer class="comment">
    <div class="user">
      <img
        class="avatar"
        :src="comment.created_by.avatar"
        :alt="comment.created_by.username + '\'s avatar'"
      />
      <span class="username">
        {{ comment.created_by.username }}
      </span>
      <span class="time-ago">
        {{ timeAgo(comment.created_at) }}
      </span>
    </div>
    <div class="comment-body">
      <div>{{ comment.content }}</div>
    </div>
  </ContentContainer>
</template>

<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import { timeAgo } from '@/services/helpers'
export default {
  components: { ContentContainer },
  props: {
    comment: {},
  },
  methods: {
    timeAgo(datetime) {
      return timeAgo(datetime)
    },
  },
}
</script>

<style scoped>
.comment {
  margin-top: 10px;
  display: flex;
}
.user {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 10px;
  margin-right: 7px;
  background-color: var(--color-background-primary);
  border-radius: 7px;
}
.avatar {
  width: 7em;
  border-radius: 7px;
}
.comment-body {
  padding: 7px;
}
.username {
  color: var(--color-secondary);
}
.time-ago {
  font-size: 0.8em;
  margin-top: 5px;
}
</style>
