import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';
import {EncryptComponent} from "./encrypt/encrypt/encrypt.component";
import {DecryptComponent} from "./encrypt/decrypt/decrypt.component";

const routes: Routes = [
  {path: 'encrypt', component: EncryptComponent},
  {path: 'decrypt', component: DecryptComponent}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
