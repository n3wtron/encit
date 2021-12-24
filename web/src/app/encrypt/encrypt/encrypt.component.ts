import { Component, OnInit } from '@angular/core';
import { Identity } from '../../identities/identity.service';
import { Friend } from '../../friends/friend.service';
import { EncryptService } from '../encrypt.service';
import * as FileSaver from 'file-saver';
import { MatSnackBar } from '@angular/material/snack-bar';
import { Clipboard } from '@angular/cdk/clipboard';
import { ErrorService } from '../../common/error.service';

@Component({
  selector: 'app-encrypt',
  templateUrl: './encrypt.component.html',
  styleUrls: ['./encrypt.component.scss'],
})
export class EncryptComponent implements OnInit {
  identity: Identity | undefined;
  friend: Friend | undefined;
  subject: string = '';
  message: string = '';
  private b64File: string | undefined;
  private fileName: string | undefined;

  constructor(
    private readonly encryptService: EncryptService,
    private readonly errorService: ErrorService,
    private readonly clipboard: Clipboard,
    private readonly _snackBar: MatSnackBar
  ) {}

  ngOnInit(): void {}

  encryptFile() {
    if (this.identity && this.friend && this.b64File) {
      this.encryptService
        .encrypt(
          this.identity!.name,
          this.friend!.name,
          this.subject || this.fileName,
          'file',
          this.b64File
        )
        .subscribe(
          (result) => {
            let blob = new Blob([result]);
            FileSaver.saveAs(blob, this.fileName + '.enc');
            this.fileName = undefined;
            this.b64File = undefined;
          },
          (error) => this.errorService.error(error)
        );
    }
  }

  encryptMessage(download?: boolean) {
    if (this.identity && this.friend && !!this.message) {
      this.encryptService
        .encrypt(
          this.identity!.name,
          this.friend!.name,
          this.subject,
          'plaintext',
          this.message
        )
        .subscribe(
          (result) => {
            if (download) {
              let blob = new Blob([result]);
              FileSaver.saveAs(blob, this.subject + '.enc');
            } else {
              if (this.clipboard.copy(result)) {
                this._snackBar.open('Copied to clipboard', undefined, {
                  duration: 1000,
                });
              }
            }
          },
          (error) => this.errorService.error(error)
        );
    }
  }

  uploadFile(fileUploadEvent: EventTarget | null) {
    if (
      fileUploadEvent &&
      fileUploadEvent instanceof HTMLInputElement &&
      fileUploadEvent.files &&
      fileUploadEvent.files.length == 1
    ) {
      let fl = fileUploadEvent.files[0];
      this.fileName = fl.name;
      let fileReader = new FileReader();
      fileReader.readAsBinaryString(fl);
      fileReader.onload = (event) => {
        this.b64File = btoa(event!.target!.result!.toString());
      };
    }
  }
}
