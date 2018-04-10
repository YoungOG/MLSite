import Route from '@ember/routing/route';
import { v4 } from 'ember-uuid'; 
import { inject as service } from '@ember/service';

export default Route.extend({
    date: service('current-date'),
    session: service('auth-service'),
    errorMessage: "",
    beforeModel() {
        if(!this.get('session').get('admin')) {
            this.transitionTo('index');
        }
    },
    model() {
        return this.store.createRecord('news-post-model');
    },
    actions: {
        post_news() {
            let new_post = this.modelFor('home.create-post');

            if(new_post.title == "" || new_post.body == "") {
                this.set('errorMessage', "Some fields are empty");
                return;
            }

            if(new_post.title.len() < 4) {
                this.set('errorMessage', "The post title is too tiny");
                return;
            }

            if(new_post.body.len() < 20) {
                this.set('errorMessage', "Post is too tiny");
                return;
            }

            new_post.set('author', this.get('session').get('username'));
            new_post.set('datetime', this.get('date').getDate());
            new_post.set('uuid', v4());
            let route = this;
            new_post.save().then(function() {
                route.transitionTo('index');
            }).catch(function(reason) {
                route.set('errorMessage', reason);
            });
        }
    }
});
