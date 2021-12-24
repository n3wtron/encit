import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';
import {EncryptComponent} from "./encrypt/encrypt/encrypt.component";
import {DecryptComponent} from "./encrypt/decrypt/decrypt.component";
import {IdentitiesComponent} from "./identities/identities/identities.component";
import {FriendsComponent} from "./friends/friends/friends.component";

const routes: Routes = [
  {path: 'encrypt', component: EncryptComponent},
  {path: 'decrypt', component: DecryptComponent},
  {path: 'identities', component: IdentitiesComponent},
  {path: 'friends', component: FriendsComponent},
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
