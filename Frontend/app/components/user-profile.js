import Component from '@ember/component';
import { inject as service } from '@ember/service';

export default Component.extend({
    uuidToUsername: service('uuid-to-username'),
    username: '',
    willRender() {
        this.get('uuidToUsername').uuidToUsername(this.get('data.uuid')).then((user) => {
            this.set('username', user);
        });
    }
});
