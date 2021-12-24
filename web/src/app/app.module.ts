import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatToolbarModule} from "@angular/material/toolbar";
import {MatIconModule} from "@angular/material/icon";
import {MatSidenavModule} from "@angular/material/sidenav";
import { IdentitySelectComponent } from './identities/identity-select/identity-select.component';
import {HttpClientModule} from "@angular/common/http";
import { EncryptComponent } from './encrypt/encrypt/encrypt.component';
import {MatListModule} from "@angular/material/list";
import {MatInputModule} from "@angular/material/input";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatSelectModule} from "@angular/material/select";
import {FriendSelectComponent} from "./friends/friend-select/friend-select.component";
import {MatButtonModule} from "@angular/material/button";
import {FormsModule} from "@angular/forms";
import {MatExpansionModule} from "@angular/material/expansion";
import {ClipboardModule} from "@angular/cdk/clipboard";
import {MatSnackBarModule} from "@angular/material/snack-bar";
import { DecryptComponent } from './encrypt/decrypt/decrypt.component';
import {MatCardModule} from "@angular/material/card";
import { ErrorDialogComponent } from './common/error-dialog/error-dialog.component';
import {MatDialogModule} from "@angular/material/dialog";
import { NewIdentityComponent } from './identities/new-identity/new-identity.component';
import { IdentitiesComponent } from './identities/identities/identities.component';
import {MatTooltipModule} from "@angular/material/tooltip";
import {MatMenuModule} from "@angular/material/menu";
import {FriendsComponent} from "./friends/friends/friends.component";
import {AddFriendComponent} from "./friends/add-friend/add-friend.component";
import {MatButtonToggleModule} from "@angular/material/button-toggle";

@NgModule({
  declarations: [
    AppComponent,
    IdentitySelectComponent,
    FriendSelectComponent,
    EncryptComponent,
    DecryptComponent,
    ErrorDialogComponent,
    NewIdentityComponent,
    IdentitiesComponent,
    AddFriendComponent,
    FriendsComponent,
  ],
  imports: [
    BrowserModule,
    HttpClientModule,
    MatToolbarModule,
    ClipboardModule,
    MatSnackBarModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    MatIconModule,
    MatSidenavModule,
    MatListModule,
    MatFormFieldModule,
    MatInputModule,
    MatSelectModule,
    MatButtonModule,
    FormsModule,
    MatExpansionModule,
    MatCardModule,
    MatDialogModule,
    MatTooltipModule,
    MatMenuModule,
    MatButtonToggleModule,
  ],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
